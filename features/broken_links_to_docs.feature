Feature: recognize/fix links to non-existing documents

  Background:
    Given file "1.md" with content:
      """
      # Title

      [two](two.md)
      """

  Scenario: checking
    When checking
    Then it prints:
      """
      1.md:3  broken link to "two.md"
      """

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md:3  broken link to "two.md"
      """
