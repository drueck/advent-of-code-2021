# Advent of Code 2021: Day 8, Part 1
# https://adventofcode.com/2021/day/8
# Usage: elixir main.exs <input-file>

defmodule DayEight do
  def run do
    lengths_of_unique_digits = [2, 4, 3, 7]

    System.argv()
    |> Enum.at(0)
    |> File.stream!()
    |> Stream.map(&String.trim/1)
    |> Stream.map(&String.split(&1, " | "))
    |> Stream.flat_map(fn [_, digits] ->
      digits
      |> String.split(" ")
      |> Enum.map(&String.length/1)
    end)
    |> Stream.filter(fn n -> n in lengths_of_unique_digits end)
    |> Enum.count()
    |> IO.inspect()
  end
end

DayEight.run()
