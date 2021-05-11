Feature: accept links to existing documents

  Background:
    Given file "1.md" with content:
      """
      # Title

      [two](2.md)
      """
    And file "2.md" with content:
      """
      # Two

      [one](1.md)
      """

  Scenario: checking
    When checking
    Then it finds no issues
    And all files are unchanged

  Scenario: pitstop
    When doing a pitstop
    Then it finds no issues
    And all files are unchanged
