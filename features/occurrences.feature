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

  Scenario: check
    When checking
    Then it prints:
      """
      2.md  missing link to "Title 1"
      """

  Scenario: fix
    When fixing
    Then it prints nothing
    And file "1.md" is unchanged
    And file "2.md" should contain:
      """
      # Two
      ### occurrences
      - [Title 1](1.md)
      """

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      2.md:2  added occurrences section
      """
