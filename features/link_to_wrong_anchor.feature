@this
Feature: recognize links to wrong anchors

  Background:
    Given file "1.md" with content:
      """
      # One

      [wrong anchor](2.md#zonk)
      <a href="2.md#zonk">wrong anchor</a>
      """
    And file "2.md" with content:
      """
      # Two

      [one](1.md)
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md:3  link to non-existing file "non-existing.md"
      1.md:4  link to non-existing file "non-existing.md"
      """
    And all files are unchanged
    And the exit code is 2

  Scenario: fix
    When fixing
    Then it finds no issues
    And all files are unchanged

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md:3  link to non-existing file "non-existing.md"
      1.md:4  link to non-existing file "non-existing.md"
      """
    And all files are unchanged
    And the exit code is 2
