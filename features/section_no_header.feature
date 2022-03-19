Feature: recognize sections with an empty header

  Background:
    Given file "1.md" with content:
      """
      # Title 1

      ###
      content
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md:3  section with empty title
      """
    And the exit code is 1

  Scenario: fix
    When fixing
    Then it finds no issues
    And all files are unchanged

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md:3  section with empty title
      """
    And all files are unchanged
    And the exit code is 1
