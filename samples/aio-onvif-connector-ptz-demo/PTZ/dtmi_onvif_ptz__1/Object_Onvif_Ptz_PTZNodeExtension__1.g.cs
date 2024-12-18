/* Code generated by Azure.Iot.Operations.ProtocolCompiler; DO NOT EDIT. */

#nullable enable

namespace PTZ.dtmi_onvif_ptz__1
{
    using System;
    using System.Collections.Generic;
    using System.Text.Json.Serialization;
    using PTZ;

    public class Object_Onvif_Ptz_PTZNodeExtension__1
    {
        /// <summary>
        /// The 'Extension' Field.
        /// </summary>
        [JsonPropertyName("Extension")]
        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public Object_Onvif_Ptz_PTZNodeExtension2__1? Extension { get; set; } = default;

        /// <summary>
        /// Detail of supported Preset Tour feature.
        /// </summary>
        [JsonPropertyName("SupportedPresetTour")]
        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public Object_Onvif_Ptz_PTZPresetTourSupported__1? SupportedPresetTour { get; set; } = default;

    }
}
