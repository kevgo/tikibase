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
    And file "unordered.md" with content:
      """
      # Test

      ### two
      [other](other.md)

      ### one
      text

      ### three
      text
      """
    And file "other.md" with content:
      """
      # Other
      [unordered](unordered.md)
      """

  Scenario: check
    When checking
    Then it prints:
      """
      unordered.md:1  sections occur in different order than specified by tikibase.json
      """
    And the exit code is 1

  Scenario: fix
    When fixing
    Then it prints:
      """
      unordered.md:1  fixed section order
      """
    And file "unordered.md" should contain:
      """
      # Test

      ### one
      text

      ### two
      [other](other.md)

      ### three
      text
      """
    And the exit code is 0

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      unordered.md:1  fixed section order
      """
    And file "unordered.md" should contain:
      """
      # Test

      ### one
      text

      ### two
      [other](other.md)

      ### three
      text
      """
    And the exit code is 0
