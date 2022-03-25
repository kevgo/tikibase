Feature: recognize unused footnote definitions

  Background:
    Given file "1.md" with content:
      """
      # Title

      ### metrics
      existing footnote[^existing]

      ```go
      result := map[^0]
      ```

      Another snippet of code that should be ignored: `map[^0]`.

      ### links

      [^existing]: existing footnote
      [^unused]: unused footnote
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md:15  unused footnote [^unused]
      """
    And all files are unchanged
    And the exit code is 1

  Scenario: fix
    When fixing
    Then it finds no issues
    And all files are unchanged

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md:15  unused footnote [^unused]
      """
    And all files are unchanged
    And the exit code is 1
