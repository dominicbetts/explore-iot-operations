/* Code generated by Azure.Iot.Operations.ProtocolCompiler; DO NOT EDIT. */

#nullable enable

namespace PTZ.dtmi_onvif_ptz__1
{
    using System;
    using System.Collections.Generic;
    using System.Text.Json.Serialization;
    using PTZ;

    public class Object_PTZConfiguration_Extension
    {
        /// <summary>
        /// The 'Extension' Field.
        /// </summary>
        [JsonPropertyName("Extension")]
        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public Object_PTZConfiguration_Extension_Extension? Extension { get; set; } = default;

        /// <summary>
        /// Optional element to configure PT Control Direction related features.
        /// </summary>
        [JsonPropertyName("PTControlDirection")]
        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public Object_PTZConfiguration_Extension_PTControlDirection? PTControlDirection { get; set; } = default;

    }
}