Feature: recognize/fix links to non-existing documents

  Background:
    Given file "1.md" with content:
      """
      # Title

      [Google](https://google.com)
      """

  Scenario: checking
    When checking
    Then it finds no issues
    And all files are unchanged
