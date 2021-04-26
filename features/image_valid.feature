Feature: accept valid images

  Background:
    Given file "1.md" with content:
      """
      # Title

      <img src="existing.png">
      <img src="existing.png" />
      ![valid image](existing.png)
      """
    And binary file "existing.png"

  Scenario: this
    When checking
    Then it finds no issues

  Scenario: pitstop
    When doing a pitstop
    Then it finds no issues