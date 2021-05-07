Feature: add occurrence sections

  Background:
    Given file "1.md" with content:
      """
      # Title 1

      ### One

      [two](2.md)
      """
    And file "2.md" with content:
      """
      # Two
      """

  Scenario: this
    When checking
    Then it prints:
      """
      2.md  missing link to 1.md
      """

  Scenario: fix
    When fixing
    Then it prints:
      """
      2.md:3  added occurrences section
      """

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      2.md:3  added occurrences section
      """
