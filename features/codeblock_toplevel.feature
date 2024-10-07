Feature: top-level codeblock

  Background:
    Given file "1.md" with content:
      """
      # Title
      [Two](2.md)

      ```go
      result := map[^0]
      ```
      """
    And file "2.md" with content:
      """
      # Two
      [One](1.md)
      """

  @this
  Scenario: check
    When checking
    Then it finds no issues
