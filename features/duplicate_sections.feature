Feature: duplicate sections

  Background:
    Given file "1.md" with content:
      """
      # Title 1

      ### One

      content

      ### One

      content
      """

  Scenario: check
    When checking
    Then it finds these errors:
      """
      1.md  duplicate section: One
      """

  Scenario: pitstop
    When doing a pitstop
    Then it finds these errors:
      """
      1.md  duplicate section: One
      """
