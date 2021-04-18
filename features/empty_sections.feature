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
    When fixing
    Then file "1.md" should contain:
      """
      # Title 1

      ### Two

      content
      """


  Scenario: pitstop
    When doing a pitstop
    Then it finds no errors
    And file "1.md" should contain:
      """
      # Title 1

      ### Two

      content
      """
