Feature: Sections without content

  Background:
    Given file "1.md" with content:
      """
      # Title 1

      ### One

      ### Two

      content

      ### Three
      """

  Scenario: check
    When checking
    Then it finds these errors:
      """
      1.md:3  section "One" has no content
      1.md:9  section "Three" has no content
      """

  Scenario: fix


  Scenario: pitstop
    When doing a pitstop
      """
      """
