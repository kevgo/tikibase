@this
Feature: link to valid anchor

  Background:
    Given file "1.md" with content:
      """
      # One

      [anchor to existing section](#later)
      <a href="#later">anchor to existing section</a>

      ### later
      text
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
