Feature: recognize/fix links to non-existing documents

  Background:
    Given file "1.md" with content:
      """
      # Title

      [Google](https://google.com)
      """

  Scenario: check
    When checking
    Then it finds no issues
    And all files are unchanged

  Scenario: fix
    When fixing
    Then it finds no issues
    And all files are unchanged

  Scenario: pitstop
    When doing a pitstop
    Then it finds no issues
    And all files are unchanged
