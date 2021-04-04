Feature: Example feature

  Scenario: An example scenario
    Given a file "foo.md" with content:
      """
      bar
      """
    When I consider what I am doing
    Then that string is now equal to "foo"
