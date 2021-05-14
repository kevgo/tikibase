Feature: Ignore files

  Background:
    Given file "tikibase.json" with content:
      """
      {
        "ignore": [
          "Makefile"
        ]
      }
      """
    And file "Makefile"

  Scenario: this
    When checking
    Then it finds no issues

  Scenario: fix
    When checking
    Then it finds no issues
    And all files are unchanged

  Scenario: pitstop
    When doing a pitstop
    Then it finds no issues
    And all files are unchanged
