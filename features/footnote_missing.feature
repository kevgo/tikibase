Feature: recognize missing footnote definitions

  Background:
    Given file "1.md" with content:
      """
      # Title
      [Two](2.md)

      ### metrics
      - existing footnote[^existing]
      - non-existing footnote[^2]
      - non-existing footnote[^non-existing]
      - non-existing footnote[^this_one_neither]

      ```go
      result := map[^0]
      ```

      Another snippet of code that should be ignored: `map[^0]`.

      ### links

      [^existing]: existing footnote
      """
    And file "2.md" with content:
      """
      # Two
      [One](1.md)
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md:6  footnote [^2] doesn't exist
      1.md:7  footnote [^non-existing] doesn't exist
      1.md:8  footnote [^this_one_neither] doesn't exist
      """
    And all files are unchanged
    And the exit code is 3

  Scenario: fix
    When fixing
    Then it finds no issues
    And all files are unchanged

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md:6  footnote [^2] doesn't exist
      1.md:7  footnote [^non-existing] doesn't exist
      1.md:8  footnote [^this_one_neither] doesn't exist
      """
    And all files are unchanged
    And the exit code is 3
