Feature: recognize missing sources

  Background:
    Given file "1.md" with content:
      """
      # Title

      ### metrics
      - 100 tons of Rust [2]

      ```go
      result := map[0]
      ```

      Also, you can do this: `map[0]`.

      ### links

      1. https://www.rust-lang.org
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md:4  missing source [2]
      """
    And all files are unchanged
    And the exit code is 1

  Scenario: fixing
    When fixing
    Then it finds no issues
    And all files are unchanged

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md:4  missing source [2]
      """
    And all files are unchanged
    And the exit code is 1
