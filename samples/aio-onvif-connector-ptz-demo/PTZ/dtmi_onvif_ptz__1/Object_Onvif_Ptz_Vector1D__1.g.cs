/* Code generated by Azure.Iot.Operations.ProtocolCompiler; DO NOT EDIT. */

#nullable enable

namespace PTZ.dtmi_onvif_ptz__1
{
    using System;
    using System.Collections.Generic;
    using System.Text.Json.Serialization;
    using PTZ;

    public class Object_Onvif_Ptz_Vector1D__1
    {
        /// <summary>
        /// The 'space' Field.
        /// </summary>
        [JsonPropertyName("space")]
        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public string? Space { get; set; } = default;

        /// <summary>
        /// The 'x' Field.
        /// </summary>
        [JsonPropertyName("x")]
        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public float? X { get; set; } = default;

    }
}