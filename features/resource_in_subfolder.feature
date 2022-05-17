Feature: Resource referenced in a subfolder

  Background:
    Given file "sub/1.md" with content:
      """
      # Title
      <img src="existing.png">
      <img src="existing.png" />
      ![valid image](existing.png)
      """
    And file "sub/existing.png"

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
