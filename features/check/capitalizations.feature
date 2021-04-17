Feature: Different section capitalizations

  Background:
    Given file "1.md" with content:
      """
      # Title 1

      ### One

      ### one
      """

  Scenario: checking
    When checking
    Then it finds these sections with mixed capitalization:
      | how it works, How it works         |
      | what is it, What is it, WHAT IS IT |


  Scenario: different capitalizations
