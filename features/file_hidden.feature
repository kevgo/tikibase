Feature: ignore hidden files

  Background:
    Given file ".prettierrc"

  Scenario: check
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
