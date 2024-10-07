Feature: indented codeblock

  Background:
    Given file "1.md" with content:
      """
      # Title
      [Two](2.md)

      - point 1

        ```go
        result := map[^0]
        ```
      """
    And file "2.md" with content:
      """
      # Two
      [One](1.md)
      """

  Scenario: check
    When checking
    Then it finds no issues
