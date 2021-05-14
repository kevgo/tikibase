Feature: ignore hidden files

  Background:
    Given binary file ".prettierrc"

  Scenario: check
    When checking
    Then it finds no issues

  Scenario: pitstop
    When doing a pitstop
    Then it finds no issues
    And all files are unchanged
