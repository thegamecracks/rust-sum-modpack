#!/usr/bin/python3
"""Produce filesize statistics from a list of workshop mods."""

# /// script
# dependencies = []
# requires-python = ">=3.11"
# ///

from __future__ import annotations

import argparse
import datetime
import json
import logging
import math
import re
import urllib.parse
import urllib.request
from contextlib import contextmanager
from dataclasses import dataclass, field
from http.client import HTTPResponse
from pathlib import Path
from typing import Any, Collection, Iterable, Iterator, Self

# https://steamapi.xpaw.me/#ISteamRemoteStorage/GetPublishedFileDetails
STEAMAPI_FILEDETAILS_URL = (
    "https://api.steampowered.com/ISteamRemoteStorage/GetPublishedFileDetails/v1/"
)
# https://steamapi.xpaw.me/#ISteamRemoteStorage/GetCollectionDetails
STEAMAPI_COLLECTION_URL = (
    "https://api.steampowered.com/ISteamRemoteStorage/GetCollectionDetails/v1/"
)
WORKSHOP_ITEM_PATTERN = re.compile(
    r"https://steamcommunity.com/sharedfiles/filedetails/\?id=(\d+)"
)

log = logging.getLogger(__name__)


def main() -> None:
    parser = argparse.ArgumentParser(
        description=__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument(
        "-s",
        "--sort",
        choices=("largest", "smallest", "title"),
        default="largest",
        help="How mods are sorted in the table",
    )
    parser.add_argument(
        "-v",
        "--verbose",
        action="count",
        default=0,
        help="Increase logging verbosity",
    )
    parser.add_argument(
        "mods",
        help="Workshop item IDs, collections, or modpack files to sum",
        nargs="+",
    )

    args = parser.parse_args()
    mods: list[str] = args.mods
    verbose: int = args.verbose

    setup_logging(verbose=verbose)

    item_ids = parse_mods_to_item_ids(mods)
    if not item_ids:
        return print("Nothing to sum")

    if args.sort == "largest":
        key = lambda item: (-item.size, item.title)
    elif args.sort == "smallest":
        key = lambda item: (item.size, item.title)
    else:
        key = lambda item: item.title

    items = get_published_file_details(list(item_ids), ignore_errors=True)
    items = expand_collection_file_details(items, ignore_errors=True)
    items = {item.id: item for item in items}
    items = sorted(items.values(), key=key)

    print("Number of mods:", len(items))

    if args.sort in ("largest", "smallest"):
        sizes = [item.size for item in items]
        total_sizes_down = cumulative_sum(sizes)
        total_sizes_up = cumulative_sum(reversed(sizes))

        print(f"  #  Total (up)  Total (down)       Size  Title")
        for i, (item, up, down) in enumerate(
            zip(items, reversed(total_sizes_up), total_sizes_down)
        ):
            size = natural_bytes(item.size)
            up = natural_bytes(up)
            down = natural_bytes(down)
            print(f"{i + 1:>3d}  {up:>10}  {down:>12}  {size:>9}  {item.title}")
    else:
        print(f"  #       Size  Title")
        for i, item in enumerate(items):
            size = natural_bytes(item.size)
            print(f"{i + 1:>3d}  {size:>9}  {item.title}")


def setup_logging(*, verbose: int) -> None:
    level = logging.DEBUG if verbose else logging.INFO
    logging.basicConfig(format="%(levelname)s: %(message)s", level=level)


def parse_mods_to_item_ids(mods: Iterable[str]) -> set[int]:
    item_ids: set[int] = set()

    unknown: list[str] = []
    for m in mods:
        try:
            item_ids.add(int(m))
        except ValueError:
            unknown.append(m)

    for m in unknown:
        path = Path(m)
        if not path.exists():
            raise ValueError(f"Unknown mod: {m}")

        item_ids |= read_modpack_item_ids(path)

    return item_ids


def read_modpack_item_ids(modpack: Path) -> set[int]:
    content = modpack.read_text("utf-8")
    return {int(m[1]) for m in WORKSHOP_ITEM_PATTERN.finditer(content)}


def cumulative_sum(nums: Iterable[int]) -> list[int]:
    sums: list[int] = []
    total = 0
    for n in nums:
        total += n
        sums.append(total)
    return sums


@dataclass(kw_only=True)
class CollectionDetails:
    id: int
    children: list[int]

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Self:
        log.debug("Parsing collection details: %s", data)

        item_id = int(data["publishedfileid"])
        if data["result"] == 9:
            raise DetailsError(f"Item ID not found: {item_id}")
        elif data["result"] != 1:
            r = data["result"]
            raise DetailsError(f"Item ID {item_id} Unexpected result code: {r}")

        return cls(
            id=item_id,
            children=[int(child["publishedfileid"]) for child in data["children"]],
        )


@dataclass(kw_only=True)
class FileDetails:
    id: int
    title: str
    description: str = field(repr=False)
    created_at: datetime.datetime
    updated_at: datetime.datetime
    size: int
    tags: list[Tag]

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Self:
        log.debug("Parsing file details: %s", data)

        item_id = int(data["publishedfileid"])
        if data["result"] == 9:
            raise DetailsError(f"Item ID not found: {item_id}")
        elif data["result"] != 1:
            r = data["result"]
            raise DetailsError(f"Item ID {item_id} Unexpected result code: {r}")

        created_at = datetime.datetime.fromtimestamp(data["time_created"]).astimezone()
        updated_at = datetime.datetime.fromtimestamp(data["time_updated"]).astimezone()
        return cls(
            id=item_id,
            title=data["title"],
            description=data["description"],
            created_at=created_at,
            updated_at=updated_at,
            size=int(data["file_size"]),
            tags=[Tag.from_dict(tag) for tag in data["tags"]],
        )


@dataclass(kw_only=True)
class Tag:
    name: str

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Self:
        return cls(name=data["tag"])


class DetailsError(Exception):
    """Raised when file details cannot be parsed from the Steam Web API."""


def expand_collection_file_details(
    items: Iterable[FileDetails],
    *,
    ignore_errors: bool,
) -> list[FileDetails]:
    collections: list[FileDetails] = []
    ret: list[FileDetails] = []
    for item in items:
        if any(tag.name == "Collection" for tag in item.tags):
            collections.append(item)
        else:
            ret.append(item)

    if collections:
        item_ids = [item.id for item in collections]
        expanded = get_collection_details(item_ids, ignore_errors=ignore_errors)
        expanded = set(id for item in expanded for id in item.children)
        expanded -= {item.id for item in ret}
        expanded = get_published_file_details(expanded, ignore_errors=ignore_errors)
        ret.extend(expanded)

    return ret


def get_collection_details(
    item_ids: Collection[int],
    *,
    ignore_errors: bool,
) -> list[CollectionDetails]:
    if not item_ids:
        return []

    data: dict[str, object] = {"collectioncount": len(item_ids)}
    for i, item in enumerate(item_ids):
        data[f"publishedfileids[{i}]"] = str(item)

    with http_post(STEAMAPI_COLLECTION_URL, data=data) as response:
        raise_for_status(response)
        payload = response.read()
        payload = json.loads(payload)

    details = payload["response"]["collectiondetails"]
    items = []
    for item in details:
        try:
            item = CollectionDetails.from_dict(item)
        except DetailsError as e:
            if not ignore_errors:
                raise
            log.warning(e)
        else:
            items.append(item)

    if not ignore_errors and len(items) != len(item_ids):
        raise DetailsError(f"Expected {len(item_ids)} items, but received {len(items)}")

    return items


def get_published_file_details(
    item_ids: Collection[int],
    *,
    ignore_errors: bool,
) -> list[FileDetails]:
    if not item_ids:
        return []

    data: dict[str, object] = {"itemcount": len(item_ids)}
    for i, item in enumerate(item_ids):
        data[f"publishedfileids[{i}]"] = str(item)

    with http_post(STEAMAPI_FILEDETAILS_URL, data=data) as response:
        raise_for_status(response)
        payload = response.read()
        payload = json.loads(payload)

    details = payload["response"]["publishedfiledetails"]
    items = []
    for item in details:
        try:
            item = FileDetails.from_dict(item)
        except DetailsError as e:
            if not ignore_errors:
                raise
            log.warning(e)
        else:
            items.append(item)

    if not ignore_errors and len(items) != len(item_ids):
        raise DetailsError(f"Expected {len(item_ids)} items, but received {len(items)}")

    return items


@contextmanager
def http_post(url: str, data: dict[str, object]) -> Iterator[HTTPResponse]:
    request = urllib.request.Request(
        url,
        data=urllib.parse.urlencode(data).encode(),
        headers={},
        method="POST",
    )

    with urllib.request.urlopen(request, timeout=5) as response:
        assert isinstance(response, HTTPResponse)
        yield response


def natural_bytes(n: int) -> str:
    if n < 1e3:
        return f"{n}B"
    elif n < 1e6:
        return f"{math.ceil(n / 1e3)}KB"
    return f"{math.ceil(n / 1e6):,d}MB"


def raise_for_status(response: HTTPResponse):
    if not 200 <= response.status < 300:
        raise RuntimeError(f"Unexpected HTTP status code: {response.status}")


if __name__ == "__main__":
    main()
