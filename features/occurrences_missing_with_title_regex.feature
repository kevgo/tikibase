Feature: add occurrence sections with a title regex

  Background:
    Given file "tikibase.json" with content:
      """
      {
        "bidiLinks": true,
        "titleRegEx": "\\((\\w+)\\)$"
      }
      """
    And file "1.md" with content:
      """
      # One

      ### section
      [Four](4.md)
      """
    And file "2.md" with content:
      """
      # [One](1.md) times two (multiplication)
      """
    And file "3.md" with content:
      """
      # Three

      ### Bar
      [One](1.md#section)
      """
    And file "4.md" with content:
      """
      # Four
      [One](1.md)
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md:4  missing link to 2.md
      1.md:4  missing link to 3.md
      """
    And the exit code is 2

  Scenario: fix
    When fixing
    Then it prints:
      """
      1.md:6  added 2.md to occurrences section
      1.md:6  added 3.md to occurrences section
      """
    And file "1.md" should contain:
      """
      # One

      ### section
      [Four](4.md)

      ### occurrences

      - [multiplication](2.md)
      - [Three](3.md)
      """
    And file "2.md" is unchanged
    And file "3.md" is unchanged
    And the exit code is 0

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md:6  added 2.md to occurrences section
      1.md:6  added 3.md to occurrences section
      """
    And file "1.md" should contain:
      """
      # One

      ### section
      [Four](4.md)

      ### occurrences

      - [multiplication](2.md)
      - [Three](3.md)
      """
    And the exit code is 0
