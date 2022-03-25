Feature: verify the ordering of content sections

  Background:
    Given file "tikibase.json" with content:
      """
      {
        "sections": [
          "one",
          "two",
          "three"
        ]
      }
      """
    And file "test.md" with content:
      """
      # Test

      ### one
      text

      ### three
      text

      ### two
      text
      """

  Scenario: check
    When checking
    Then it prints:
      """
      test.md:6  sections occur in different order than specified by tikibase.json
      """
    And the exit code is 1

  Scenario: fix
    When fixing
    Then it prints:
      """
      test.md:6  fixed section order
      """
    And file "test.md" should contain:
      """
      # Test

      ### one
      text

      ### two
      text

      ### three
      text
      """
    And the exit code is 0

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      test.md:6  fixed section order
      """
    And file "test.md" should contain:
      """
      # Test

      ### one
      text

      ### two
      text

      ### three
      text
      """
    And the exit code is 0
