Feature: accept links to existing directories

  Background:
    Given file "1.md" with content:
      """
      # Title

      [sub](sub/)
      """
    And file "sub/2.md" with content:
      """
      # Two

      [one](../1.md)
      """

  @this
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
