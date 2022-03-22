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
      1.md:7  document contains multiple "One" sections
      """
    And all files are unchanged

  Scenario: fix
    When fixing
    Then it finds no issues
    And all files are unchanged

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md:7  document contains multiple "One" sections
      """
    And all files are unchanged
