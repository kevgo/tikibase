Feature: recognize sections with inconsistent heading level

  Background:
    Given file "1.md" with content:
      """
      # One

      ### alpha
      [Two](2.md)
      """
    And file "2.md" with content:
      """
      # Two

      ##### alpha
      [One](1.md)
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md:3  inconsistent heading level
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
      1.md:3  inconsistent heading level
      """
    And all files are unchanged
    And the exit code is 1
