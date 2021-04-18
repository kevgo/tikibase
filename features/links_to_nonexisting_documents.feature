Feature: recognize/fix links to non-existing documents

  Background:
    Given file "1.md" with content:
      """
      # Title

      [two](two.md)
      """

  Scenario: checkthis
    When checking
    Then it prints:
      """
      1.md:3  link to non-existing document "two.md"
      """
