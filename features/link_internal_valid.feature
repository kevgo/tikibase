Feature: accept valid internal links

  Background:
    Given file "1.md" with content:
      """
      # Title

      [two](2.md)
      """
    And file "2.md" with content:
      """
      # Two
      """

  Scenario: checking
    When checking
    Then it finds no issues

  Scenario: pitstop
    When doing a pitstop
    Then it finds no issues
