Feature: recognize/fix links to non-existing documents

  Background:
    Given file "1.md" with content:
      """
      # Title

      [zonk](non-existing.md)
      <a href="non-existing">zonk</a>
      """

  Scenario: checking
    When checking
    Then it prints:
      """
      1.md:3  broken link to "non-existing.md"
      1.md:4  broken link to "non-existing.md"
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
      1.md:3  broken link to "non-existing.md"
      1.md:4  broken link to "non-existing.md"
      """
    And all files are unchanged
    And the exit code is 1
