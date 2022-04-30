@this
Feature: links to a file in a subfolder

  Background:
    Given file "1.md" with content:
      """
      # One
      [Two](sub/folder/2.md)
      """
    And file "sub/folder/2.md" with content:
      """
      # Two
      [One](../../1.md)
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
