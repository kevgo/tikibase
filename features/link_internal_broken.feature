Feature: recognize/fix links to non-existing documents

  Background:
    Given file "1.md" with content:
      """
      # Title

      [zonk](non-existing.md)
      """

  Scenario: checking
    When checking
    Then it prints:
      """
      1.md:3  broken link to "non-existing.md"
      """
    And all files are unchanged

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md:3  broken link to "non-existing.md"
      """
    And all files are unchanged
