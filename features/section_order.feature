Feature: verify the ordering of content sections

  Background:
    Given file "tikibase.json" with content:
      """
      {
        "section_names": [
          "one",
          "two",
          "three"
        ]
      }
      """
    And file "test.md" with content:
      """
      # Test

      ### two
      text

      ### one
      text

      ### three
      text
      """

  Scenario: check
    When checking
    Then it prints:
      """
      test.md  wrong section order
      """
    And the exit code is 1

  Scenario: fix
    When fixing
    Then it prints:
      """
      test.md  fixed section order
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
      test.md  fixed section order
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
