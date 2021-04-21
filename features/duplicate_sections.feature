Feature: recognize/fix duplicate sections

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
    Then it prints:
      """
      1.md  duplicate section: One
      """

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md  duplicate section: One
      """