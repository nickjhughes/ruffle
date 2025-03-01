package {
    [Ruffle(InstanceAllocator)]
    [Ruffle(CallHandler)]
    public final dynamic class XML {
        import __ruffle__.stub_method;

        AS3 function normalize(): XML {
            stub_method("XML", "normalize");
            return this;
        }
        
        AS3 static function setSettings(settings:Object): void {
            stub_method("XML", "setSettings");
        }

        AS3 static function settings():Object {
            stub_method("XML", "settings");

            return {
                ignoreComments: true,
                ignoreProcessingInstructions: true,
                ignoreWhitespace: true,
                prettyIndent: 2,
                prettyPrinting: true
            };
        }

        public function XML(value:* = undefined) {
            this.init(value);
        }

        private native function init(value:*):void;

        AS3 native function hasComplexContent():Boolean;
        AS3 native function hasSimpleContent():Boolean;
        AS3 native function name():Object;
        AS3 native function namespace(prefix:String = null):*;
        AS3 native function localName():Object;
        AS3 native function toXMLString():String;
        AS3 native function child(name:Object):XMLList;
        AS3 native function childIndex():int;
        AS3 native function children():XMLList;
        AS3 native function copy():XML;
        AS3 native function parent():*;
        AS3 native function elements(name:*):XMLList;
        AS3 native function attributes():XMLList;
        AS3 native function attribute(name:*):XMLList;
        AS3 native function nodeKind():String;
        AS3 native function appendChild(child:Object):XML;
        AS3 native function descendants(name:Object = "*"):XMLList;
        AS3 native function text():XMLList;
        AS3 native function toString():String;
        AS3 native function length():int;

        prototype.hasComplexContent = function():Boolean {
            var self:XML = this;
            return self.AS3::hasComplexContent();
        }

        prototype.hasSimpleContent = function():Boolean {
            var self:XML = this;
            return self.AS3::hasSimpleContent();
        }

        prototype.name = function():Object {
            var self:XML = this;
            // NOTE - `self.name()` should be sufficient here (and in all of the other methods)
            // However, asc.jar doesn't resolve the 'AS3' namespace when I do
            // 'self.name()' here, which leads to the prototype method invoking
            // itself, instead of the AS3 method.
            return self.AS3::name();
        };

        prototype.namespace = function(prefix:String = null):* {
            var self:XML = this;
            return self.AS3::namespace(prefix);
        }

        prototype.localName = function():Object {
            var self:XML = this;
            return self.AS3::localName();
        };

        prototype.toXMLString = function():String {
            var self:XML = this;
            return self.AS3::toXMLString();
        };

        prototype.child = function(name:Object):XMLList {
            var self:XML = this;
            return self.AS3::child(name);
        };

        prototype.childIndex = function():XMLList {
            var self:XML = this;
            return self.AS3::childIndex();
        };

        prototype.children = function():XMLList {
            var self:XML = this;
            return self.AS3::children();
        };

        prototype.copy = function():XML {
            var self:XML = this;
            return self.AS3::copy();
        }

        prototype.parent = function():* {
            var self:XML = this;
            return self.AS3::parent();
        };

        prototype.elements = function(name:*):XMLList {
            var self:XML = this;
            return self.AS3::elements(name);
        }

        prototype.toString = function():String {
            if (this === prototype) {
                return "";
            }
            var self:XML = this;
            return self.AS3::toString();
        };

        prototype.attributes = function():XMLList {
            var self:XML = this;
            return self.AS3::attributes();
        };

        prototype.attribute = function(name:*):XMLList {
            var self:XML = this;
            return self.AS3::attribute(name);
        };

        prototype.nodeKind = function():String {
            var self:XML = this;
            return self.AS3::nodeKind();
        };

        prototype.appendChild = function(child:Object):XML {
            var self:XML = this;
            return self.AS3::appendChild(child);
        };

        prototype.descendants = function(name:Object):XMLList {
            var self:XML = this;
            return self.AS3::descendants(name);
        };

        prototype.text = function():XMLList {
            var self:XML = this;
            return self.AS3::text();
        };
        
        prototype.normalize = function():XML {
            var self:XML = this;
            return self.AS3::normalize();
        };

        prototype.length = function():int {
            var self:XML = this;
            return self.AS3::length();
        }

        public static const length:int = 1;
    }
}
