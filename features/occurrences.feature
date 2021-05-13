Feature: add occurrence sections

  Background:
    Given file "1.md" with content:
      """
      # Title 1

      ### section 1

      text
      """
    And file "2.md" with content:
      """
      # Title 2

      ### Foo

      [one](1.md)
      """
    And file "3.md" with content:
      """
      # Title 3

      ### Bar

      [one](1.md#section-1)
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md  missing link to 2.md, 3.md
      """
    And the exit code is 1

  Scenario: fix
    When fixing
    Then it prints:
      """
      1.md:7  added occurrences section
      """
    And file "1.md" should contain:
      """
      # Title 1

      ### section 1

      text

      ### occurrences

      - [Title 2](2.md)
      - [Title 3](3.md)
      """
    And file "2.md" is unchanged
    And file "3.md" is unchanged
    And the exit code is 0

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md:7  added occurrences section
      """
    And the exit code is 0
