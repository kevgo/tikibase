Feature: recognize/fix links to non-existing documents

  Background:
    Given file "1.md" with content:
      """
      # Title

      [zonk](non-existing.md)
      <a href="non-existing.md">zonk</a>
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md:3  link to non-existing file "non-existing.md"
      1.md:4  link to non-existing file "non-existing.md"
      """
    And all files are unchanged
    And the exit code is 2

  Scenario: fix
    When fixing
    Then it finds no issues
    And all files are unchanged

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md:3  link to non-existing file "non-existing.md"
      1.md:4  link to non-existing file "non-existing.md"
      """
    And all files are unchanged
    And the exit code is 2
