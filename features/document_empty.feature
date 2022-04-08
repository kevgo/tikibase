Feature: Find empty documents

  Background:
    Given file "1.md" with content:
      """
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md  no content
      """
    And all files are unchanged
    And the exit code is 1

  Scenario: fix
    When fixing
    Then it prints:
      """
      1.md  no content
      """
    And all files are unchanged

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md  no content
      """
    And all files are unchanged
    And the exit code is 1
