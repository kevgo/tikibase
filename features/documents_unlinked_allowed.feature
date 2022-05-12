Feature: allow standalone documents

  Background:
    Given file "tikibase.json" with content:
      """
      {
        "standaloneDocs": true
      }
      """
    And file "1.md" with content:
      """
      # One
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
