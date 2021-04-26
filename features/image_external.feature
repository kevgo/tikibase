Feature: ignore external images

  Background:
    Given file "1.md" with content:
      """
      # Title

      <img src="https://google.com/foo.png">
      <img src="https://google.com/foo.png" />
      ![broken image](https://google.com/foo.png)
      """

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
