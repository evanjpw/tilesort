"""Type stubs for tilesort package."""

from typing import Callable, TypeVar, overload

_T = TypeVar("_T")
_KT = TypeVar("_KT")

@overload
def sort(
    list: list[_T],
    *,
    key: None = None,
    reverse: bool = False,
) -> None: ...
@overload
def sort(
    list: list[_T],
    *,
    key: Callable[[_T], _KT],
    reverse: bool = False,
) -> None: ...
@overload
def sorted(
    list: list[_T],
    *,
    key: None = None,
    reverse: bool = False,
) -> list[_T]: ...
@overload
def sorted(
    list: list[_T],
    *,
    key: Callable[[_T], _KT],
    reverse: bool = False,
) -> list[_T]: ...

__all__ = ["sort", "sorted"]
