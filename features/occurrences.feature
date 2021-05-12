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
      1.md  missing link to "Title 2"
      1.md  missing link to "Title 3"
      """

  Scenario: this
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

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md:7  added occurrences section
      """
