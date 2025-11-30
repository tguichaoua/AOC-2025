const YEAR = 2025
const MAX_DAY = 12

def get-day []: nothing -> int {
    let now = date now | into record
    if ($now | get year) != $YEAR or ($now | get month) != 12 or ($now | get day) > $MAX_DAY {
        error make --unspanned {msg: $'The current date is not a valid AOC day for the year ($YEAR)'}
    }
    $now | get day
}

export def main [day?: int] {
    let day = $day | default {get-day}

    ^cargo scaffold $day
    ^cargo download $day
    start https://adventofcode.com/($YEAR)/day/($day)
}
