Feature: Sections with different capitalization

  Background:
    Given file "1.md" with content:
      """
      # Title 1

      ### One

      ### one
      """
    And file "2.md" with content:
      """
      # Title 2

      ### ONE
      """

  Scenario: check
    When checking
    Then it finds these errors:
      """
      mixed section capitalization: one, One, ONE
      """

  Scenario: fix


  Scenario: pitstop
