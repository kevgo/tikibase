Feature: link to valid anchor in another document

  Background:
    Given file "1.md" with content:
      """
      # One
      [link to existing section in 2.md](2.md#section)
      """
    And file "2.md" with content:
      """
      # Two
      ### section
      [backlink](1.md)
      """

  Scenario: check
    When checking
    Then it finds no issues

  Scenario: fix
    When fixing
    Then it finds no issues
    And all files are unchanged

  Scenario: pitstop
    When doing a pitstop
    Then it finds no issues
