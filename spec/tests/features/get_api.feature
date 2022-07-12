Feature: Get Api

    Api need client,gvk,ar
    @serial
    Scenario: USE DEFAULT ENV TO REQ API
        Given No set var to environment, use default ApiWorld
        Then I get api
# @serial
# Scenario: USE ENV(Port,Image) TO REQ API
#     Given Port and Image
#         | port | image         |
#         | 8080 | hzgood/mycode |
#         | 8081 | alpine        |
#     Then I get api
