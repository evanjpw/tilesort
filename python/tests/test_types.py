"""Test file to verify type hints work correctly with mypy and pytest."""

import tilesort


def test_basic_sort_types() -> None:
    """Test that basic sort operations have correct types."""
    numbers: list[int] = [3, 1, 4, 1, 5]
    tilesort.sort(numbers)
    assert numbers == [1, 1, 3, 4, 5]
    # mypy should infer numbers is list[int]


def test_sort_with_key_function() -> None:
    """Test that sort with key function maintains correct types."""
    words: list[str] = ["elephant", "cat", "dog"]
    tilesort.sort(words, key=len)
    assert words == ["cat", "dog", "elephant"]
    # mypy should infer words is list[str]


def test_sorted_return_type() -> None:
    """Test that sorted() returns correct type."""
    data: list[int] = [5, 2, 8, 1]
    result: list[int] = tilesort.sorted(data)
    assert result == [1, 2, 5, 8]
    assert data == [5, 2, 8, 1]  # Original unchanged
    # mypy should infer result is list[int]


def test_reverse_parameter_types() -> None:
    """Test that reverse parameter works with correct types."""
    numbers: list[int] = [1, 2, 3, 4, 5]
    tilesort.sort(numbers, reverse=True)
    assert numbers == [5, 4, 3, 2, 1]


def test_sorted_with_key_and_reverse() -> None:
    """Test sorted with both key and reverse parameters."""
    words: list[str] = ["a", "cat", "dog", "elephant"]
    sorted_words: list[str] = tilesort.sorted(words, key=len, reverse=True)
    assert sorted_words == ["elephant", "cat", "dog", "a"]
    # mypy should infer sorted_words is list[str]


def test_custom_objects_with_key() -> None:
    """Test that custom objects work correctly with type hints."""

    class Person:
        def __init__(self, name: str, age: int) -> None:
            self.name = name
            self.age = age

        def __eq__(self, other: object) -> bool:
            if not isinstance(other, Person):
                return NotImplemented
            return self.name == other.name and self.age == other.age

        def __repr__(self) -> str:
            return f"Person({self.name!r}, {self.age})"

    people: list[Person] = [
        Person("Alice", 30),
        Person("Bob", 25),
        Person("Charlie", 35),
    ]

    # Sort by age
    tilesort.sort(people, key=lambda p: p.age)
    assert people[0].age == 25
    assert people[1].age == 30
    assert people[2].age == 35

    # Sorted by name
    sorted_people: list[Person] = tilesort.sorted(people, key=lambda p: p.name)
    assert sorted_people[0].name == "Alice"
    assert sorted_people[1].name == "Bob"
    assert sorted_people[2].name == "Charlie"
    # mypy should infer sorted_people is list[Person]


def test_generic_type_preservation() -> None:
    """Test that generic types are preserved correctly."""
    # Test with tuples
    tuples: list[tuple[str, int]] = [("c", 3), ("a", 1), ("b", 2)]
    sorted_tuples: list[tuple[str, int]] = tilesort.sorted(tuples)
    assert sorted_tuples == [("a", 1), ("b", 2), ("c", 3)]

    # Test with key on tuples
    tilesort.sort(tuples, key=lambda x: x[1])
    assert tuples == [("a", 1), ("b", 2), ("c", 3)]
