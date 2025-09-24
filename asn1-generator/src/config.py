config = {
    "ngap": [
        {
            "operation": "derive-macros",
            "additional_traits": ["Eq", "PartialEq", "Hash", "valuable::Valuable"],
            "asn1-fields": [
                "GlobalRanNodeId",
                "GlobalNgEnbId",
                "GlobalN3IwfId",
                "GlobalTngfId",
                "GlobalTwifId",
                "GlobalWAgfId",
                "GlobalGnbId",
                "NgEnbId",
                "N3IwfId",
                "TngfId",
                "TwifId",
                "WAgfId",
                "GnbId",
                "PlmnIdentity",
            ],
        },
        {
            "operation": "derive-macros",
            "additional_traits": ["Eq", "PartialEq"],
            "asn1-fields": [
                "FiveGSTmsi"
            ],
        },
    ]
}
